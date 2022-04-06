function submit() {
  let username = document.querySelector("[username]");
  let password = document.querySelector("[password]");

  document.querySelector("[submit]").classList.add("is-loading");
  username.disabled = true;
  password.disabled = true;

  fetch("/auth/login", {
    method: "POST",
    body: JSON.stringify({
      username: username.value,
      password: password.value,
    }),
  })
    .then((d) => d.json())
    .then((d) => {
      console.log(d);
      if ("error" in d) {
        bulmaToast.toast({
          message: d.error,
          duration: 5000,
          type: "is-danger",
          dismissible: true,
          animate: { in: "fadeIn", out: "fadeOut" },
        });
        document.querySelector("[submit]").classList.remove("is-loading");
        username.disabled = false;
        password.disabled = false;
        return;
      }

      bulmaToast.toast({
        message: "Sucess!",
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
          id: d.userId
        })
      );
      let params = new URLSearchParams(window.location.search);
      let path = params.get("return");

      if (path != null) window.location = path;
      else window.location = "/";
    });
}
