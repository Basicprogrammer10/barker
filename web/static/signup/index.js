function submit() {
  let username = document.querySelector("[username]");
  let password = document.querySelector("[password]");
  let confirmPassword = document.querySelector("[confirm-password]");

  if (password.value.length < 5) return error("Password too short (>=5)");

  if (confirmPassword.value != password.value)
    return error("Passwords dont match!");

  document.querySelector("[submit]").classList.add("is-loading");
  username.disabled = true;
  password.disabled = true;

  fetch("/auth/signup", {
    method: "POST",
    body: JSON.stringify({
      username: username.value,
      password: password.value,
    }),
  })
    .then((d) => d.json())
    .then((d) => {
      if ("error" in d) {
        document.querySelector("[submit]").classList.remove("is-loading");
        username.disabled = false;
        password.disabled = false;
        return error(d.error);
      }

      bulmaToast.toast({
        message: "Account Created",
        duration: 5000,
        type: "is-success",
        dismissible: true,
        animate: { in: "fadeIn", out: "fadeOut" },
      });

      fetch("/auth/login", {
        method: "POST",
        body: JSON.stringify({
          username: username.value,
          password: password.value,
        }),
      })
        .then((d) => d.json())
        .then((d) => {
          if ("error" in d) {
            document.querySelector("[submit]").classList.remove("is-loading");
            username.disabled = false;
            password.disabled = false;
            return error(d.error);
          }

          bulmaToast.toast({
            message: "Signed in",
            duration: 5000,
            type: "is-success",
            dismissible: true,
            animate: { in: "fadeIn", out: "fadeOut" },
          });

          localStorage.setItem(
            "session",
            JSON.stringify({
              token: d.token,
              username: username.value,
            })
          );
          let params = new URLSearchParams(window.location.search);
          let path = params.get("return");

          if (path != null) window.location = path;
          else window.location = "/";
        });
    });
}

function error(e) {
  bulmaToast.toast({
    message: e,
    duration: 5000,
    type: "is-danger",
    dismissible: true,
    animate: { in: "fadeIn", out: "fadeOut" },
  });
}
