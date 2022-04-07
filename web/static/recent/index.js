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
      bulmaToast.toast({
        message: "bark created",
        duration: 5000,
        type: "is-success",
        dismissible: true,
        animate: { in: "fadeIn", out: "fadeOut" },
      });
    });
}

// ==================

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

// function setLike(session, id, likeState) {
//   let setState = !likeState;
// }

// ==================

function checkNewLen() {
  let value = document.querySelector("[new-text]").value;
  let chars = value.length;

  if (chars > 256) {
    document.querySelector("[new-text]").value = value.substr(0, 256);
    chars = 256;
  }

  document.querySelector("[char-count]").innerHTML = `${chars}/256${
    chars >= 256 ? "!" : ""
  }`;
}

window.addEventListener("load", () => {
  const textBox = document.querySelector("[new-text]");
  textBox.style.height = "80px";
  textBox.style.overflowY = "hidden";

  textBox.addEventListener("input", (e) => {
    localStorage.setItem("text", e.target.value);
    textBox.style.height = "auto";
    textBox.style.height = textBox.scrollHeight + "px";
  });
});
