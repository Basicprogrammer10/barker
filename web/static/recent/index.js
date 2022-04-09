let ogLikes = {};

function data() {
  return {
    loading: true,
    newBark: false,
    session: safeJsonParse(localStorage.getItem("session")),
    barks: [],
  };
}

function sendBark() {
  let value = document.querySelector("[new-text]").value;
  let session = JSON.parse(localStorage.getItem("session"));

  fetch("/api/new", {
    method: "POST",
    body: JSON.stringify({
      token: session.token,
      message: value,
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

      document.body.dispatchEvent(new CustomEvent("refresh-barks"));
      document.body.dispatchEvent(new CustomEvent("bark-send"));
      document.querySelector("[new-text]").value = '';
      bulmaToast.toast({
        message: "bark created",
        duration: 5000,
        type: "is-success",
        dismissible: true,
        animate: { in: "fadeIn", out: "fadeOut" },
      });
    });
}

async function getRecentBarks(session) {
  let d = await fetch(
    "/api/recent" + (session == null ? "" : `?token=${session.token}`)
  ).then((d) => d.json());

  if ("error" in d)
    return bulmaToast.toast({
      message: d.error,
      duration: 5000,
      type: "is-danger",
      dismissible: true,
      animate: { in: "fadeIn", out: "fadeOut" },
    });

  return d;
}

function setLike(session, id, barks) {
  let bark = barks.find((e) => e.id == id);
  if (!(bark.id in ogLikes)) ogLikes[bark.id] = [bark.likeing, bark.likes];
  bark.likeing = !bark.likeing;

  fetch("/api/like", {
    method: "POST",
    body: JSON.stringify({
      token: session.token,
      message: id,
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

  let likes = ogLikes[bark.id][1];
  if (bark.likeing && !ogLikes[bark.id][0]) likes += 1;
  if (!bark.likeing && ogLikes[bark.id][0]) likes -= 1;
  bark.likes = likes;
}
