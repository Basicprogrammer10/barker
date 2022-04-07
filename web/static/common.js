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

function deleteBark(token, id) {
  console.log(token, id);
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

function genAvatarImage(seed) {
  if (seed.username == "darren")
    return '<img class="avatar" src="/img/beans.png" />';
  return `<img class="avatar" src="https://avatars.dicebear.com/api/adventurer-neutral/${seed.id}.svg" />`;
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
