SELECT content,
      barks.date,
      users.id,
      users.username,
      deleted,
      count(iif(likes.bark_id == barks.id, 1, NULL))
FROM   barks
JOIN   users
ON     barks.author_id = users.id
JOIN   likes
WHERE  barks.id = ?;
