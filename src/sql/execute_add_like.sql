INSERT
or     IGNORE
into   likes
      (
             bark_id,
             user_id,
             date
      )
      VALUES
      (
             ?,
             ?,
             strftime('%s','now')
      );
