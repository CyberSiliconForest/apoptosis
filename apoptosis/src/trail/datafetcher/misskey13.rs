
const USER_FETCH_QUERY: &str = """
SELECT
  "user".id AS id,
  "user".username AS username,
  "user_keypair"."privateKey" AS private_key
FROM "user"
INNER JOIN "user_keypair" ON "user_keypair"."userId" = "user".id
WHERE "user"."isDeleted" IS FALSE
  AND "user"."isSuspended" IS FALSE
ORDER BY "user".id
LIMIT ? OFFSET ?;
"""

//