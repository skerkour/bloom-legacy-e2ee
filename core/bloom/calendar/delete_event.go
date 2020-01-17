package calendar

import (
	"gitlab.com/bloom42/bloom/core/bloom/kernel"
	"gitlab.com/bloom42/bloom/core/db"
)

func DeleteEvent(params DeleteEventParams) (kernel.Empty, error) {
	ret := kernel.Empty{}

	stmt, err := db.DB.Prepare("DELETE FROM calendar_events WHERE id = ?")
	if err != nil {
		return ret, err
	}
	defer stmt.Close()

	_, err = stmt.Exec(&params.ID)

	return ret, err
}