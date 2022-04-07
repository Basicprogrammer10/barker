let ogLiked = null;

function data() {
  return {
    loading: true,
    bark: null,
    error: null,
    session: safeJsonParse(localStorage.getItem("session")),
  };
}

function getQueryId() {
  let params = new URLSearchParams(window.location.search);
  return params.get("id");
}

async function getBarkFromId(id, session) {
  let data = {
    id,
  };

  if (session != null) data.token = session.token;

  let res = await (
    await fetch("/api/get", {
      method: "POST",
      body: JSON.stringify(data),
    })
  ).json();

  if ("error" in res) return [null, res.error];

  return [res, null];
}

function setLike(session, bark) {
  if (ogLiked == null) ogLiked = [bark[0].likeing, bark[0].likes];
  bark = bark[0];
  bark.likeing = !bark.likeing;

  fetch("/api/like", {
    method: "POST",
    body: JSON.stringify({
      token: session.token,
      message: bark.id,
      state: bark.likeing,
    }),
  })
    .then((d) => d.json())
    .then((d) => {
      if ("error" in d)
        return bulmaToast.toast({
          message: d.error,
          duration: 5000,
          type: "is-danger",
          dismissible: true,
          animate: { in: "fadeIn", out: "fadeOut" },
        });
    });

  let likes = ogLiked[1];
  if (bark.likeing && !ogLiked[0]) likes = likes + 1;
  if (!bark.likeing && ogLiked[0]) likes = likes - 1;
  bark.likes = likes;
}
