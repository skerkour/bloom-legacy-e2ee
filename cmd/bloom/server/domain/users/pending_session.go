package users

import (
	"time"

	"gitlab.com/bloom42/gobox/uuid"
)

// PendingSession are created when 2fa is actived
type PendingSession struct {
	ID             uuid.UUID `db:"id"`
	CreatedAt      time.Time `db:"created_at"`
	VerifiedAt     time.Time `db:"verified_at"`
	Hash           []byte    `db:"hash"`
	Salt           []byte    `db:"salt"`
	FailedAttempts int64     `db:"failed_attempts"`

	UserID uuid.UUID `db:"user_id"`
}
