let ogLiked = null;
let queryId = null;

function data() {
  return {
    session: safeJsonParse(localStorage.getItem("session")),
    loading: true,
    bark: null,
    error: null,
    comments: [],
  };
}

function getQueryId() {
  if (queryId != null) return queryId;

  let params = new URLSearchParams(window.location.search);
  let id = params.get("id");
  queryId = id;
  return id;
}

async function getComments(id) {
  let res = await (await fetch(`/api/comments/${id}`)).json();

  if ("error" in res)
    return err(d);

  console.log(res);
  return res;
}

function sendComment(session) {
  let text = document.querySelector('[comment-text]');

  fetch("/api/comment", {
    method: "POST",
    body: JSON.stringify({
      token: session.token,
      id: getQueryId(),
      comment: text.value,
    }),
  })
    .then((d) => d.json())
    .then((d) => {
      if ("error" in d)
        return err(d);
        text.value = '';
        document.body.dispatchEvent(new CustomEvent("refresh-comments"));
    });
}

function deleteComment(session, commentId) {

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
        return err(d);
    });

  let likes = ogLiked[1];
  if (bark.likeing && !ogLiked[0]) likes = likes + 1;
  if (!bark.likeing && ogLiked[0]) likes = likes - 1;
  bark.likes = likes;
}

function err(d) {
  bulmaToast.toast({
    message: d.error,
    duration: 5000,
    type: "is-danger",
    dismissible: true,
    animate: { in: "fadeIn", out: "fadeOut" },
  });
}
