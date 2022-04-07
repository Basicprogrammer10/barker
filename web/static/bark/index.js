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

async function getBarkFromId(id) {
  let res = await (
    await fetch("/api/get", {
      method: "POST",
      body: JSON.stringify({
        id,
      }),
    })
  ).json();

  if ("error" in res)
    return [
      null,
      res.error == "Bark deleted" ? "bark deleted" : "bark not found",
    ];

  return [res, null];
}
