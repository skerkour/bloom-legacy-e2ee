package objects

import (
	"context"
	"time"

	"gitlab.com/bloom42/bloom/core/domain/kernel"
)

// Sync sync the local data with the pod (pull + conflict resolution + push)
// it should only be manually called after signing in to sync data
func Sync() (err error) {
	syncyingMutext.Lock()
	defer syncyingMutext.Unlock()

	// sync only if user is authenticated
	if kernel.Me != nil {
		err = pull()
		if err != nil {
			return
		}
		err = push()
	}
	return
}

// backgroundSync is a worker that do background sync
func backgroundSync(ctx context.Context) {
	ticker := time.NewTicker(4 * time.Second)

	for {
		select {
		case <-ctx.Done():
			// cleanup
			ticker.Stop()
			break
		case <-ticker.C:
			Sync()
		case <-SyncChan:
			Sync()
		}
	}
}
