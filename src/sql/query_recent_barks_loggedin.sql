SELECT content,
      barks.date,
      barks.id,
      users.id,
      users.username,
      (SELECT Count(*)
       FROM   likes
       WHERE  bark_id = barks.id),
      (SELECT Count(*)
       FROM   likes
       WHERE  bark_id = barks.id
              AND user_id = ?),
      (SELECT Count(*)
       FROM   comments
       WHERE  bark_id = barks.id)
FROM   barks
      JOIN users
        ON barks.author_id = users.id
WHERE  deleted = false
ORDER  BY barks.date DESC
LIMIT  ?;
