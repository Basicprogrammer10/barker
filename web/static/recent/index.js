const TIME_UNITS = [
  ["second", 60],
  ["minute", 60],
  ["hour", 24],
  ["day", 30],
  ["month", 12],
  ["year", 0],
];

function safeJsonParse(json) {
  try {
    return JSON.parse(json);
  } catch {
    return null;
  }
}

function data() {
  return {
    loading: true,
    newBark: false,
    session: safeJsonParse(localStorage.getItem("session")),
    barks: [],
  };
}

function logout(session) {
  fetch("/auth/logout", {
    method: "POST",
    body: JSON.stringify({
      token: session.token,
    }),
  })
    .then((d) => d.json())
    .then((d) => {
      if ("error" in d) {
        bulmaToast.toast({
          message: d.error,
          duration: 5000,
          type: "is-danger",
          dismissible: true,
          animate: { in: "fadeIn", out: "fadeOut" },
        });
      }

      localStorage.removeItem("session");
      window.location = "/";
    });
}

function genAvatarImage(seed) {
  if (seed.username == "darren")
    return '<img class="avatar" src="/img/beans.png" />';
  return `<img class="avatar" src="https://avatars.dicebear.com/api/adventurer-neutral/${seed.id}.svg" />`;
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
      document.querySelector("[submit]").classList.remove("is-loading");
      bulmaToast.toast({
        message: "Sucess!",
        duration: 5000,
        type: "is-success",
        dismissible: true,
        animate: { in: "fadeIn", out: "fadeOut" },
      });
    });
}

function deleteBark(token, id) {
  if (!confirm("are you sure you want to delete this bark?")) return;

  fetch("/api/delete", {
    method: "POST",
    body: JSON.stringify({
      token: token,
      message: id,
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
      bulmaToast.toast({
        message: "message deleted",
        duration: 5000,
        type: "is-success",
        dismissible: true,
        animate: { in: "fadeIn", out: "fadeOut" },
      });
    });
}

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

function epochTime(time) {
  time = new Date().getTime() / 1000 - time;
  for (let e = 0; e < TIME_UNITS.length; e++) {
    const i = TIME_UNITS[e];

    if (i[1] == 0 || time < i[1]) {
      time = Math.round(time);
      return `${time} ${i[0]}${time > 1 ? "s" : ""} ago`;
    }

    time /= i[1];
  }

  return `${Math.round(time)} years ago`;
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
