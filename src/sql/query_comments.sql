SELECT comments.id,
      user_id,
      content,
      username,
      comments.date
FROM   comments
      JOIN users
        ON user_id = users.id
WHERE  deleted = false
      AND bark_id = ?
ORDER  BY comments.date DESC;
