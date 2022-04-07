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
