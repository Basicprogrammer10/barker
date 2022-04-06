const TIME_UNITS = [
  ["second", 60],
  ["minute", 60],
  ["hour", 24],
  ["day", 30],
  ["month", 12],
  ["year", 0],
];

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
      return `${time} ${i[0]}${time > 1 ? "s" : ""}`;
    }

    time /= i[1];
  }

  return `${Math.round(time)} years`;
}
