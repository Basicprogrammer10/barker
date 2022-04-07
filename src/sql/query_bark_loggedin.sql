SELECT content,
      barks.date,
      users.id,
      users.username,
      deleted,
      (SELECT Count(*)
       FROM   likes
       WHERE  bark_id = barks.id),
      (SELECT Count(*)
       FROM   likes
       WHERE  bark_id = barks.id
              AND user_id = ?)
FROM   barks
      JOIN users
        ON barks.author_id = users.id
WHERE  barks.id = ?;  
