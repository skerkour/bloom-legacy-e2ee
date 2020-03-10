package notes

import (
	"gitlab.com/bloom42/bloom/core/db"
)

func ListNotes() (Notes, error) {
	ret := Notes{Notes: []Note{}}

	rows, err := db.DB.Query(`SELECT id, createdAt, updatedAt, archivedAt, title, body, color, isPinned
		FROM notes WHERE archivedAt IS NULL ORDER BY isPinned DESC, createdAt ASC`)
	if err != nil {
		return ret, err
	}
	defer rows.Close()

	for rows.Next() {
		var note Note
		err := rows.Scan(&note.ID, &note.CreatedAt, &note.UpdatedAt, &note.ArchivedAt, &note.Title, &note.Body, &note.Color, &note.IsPinned)
		if err != nil {
			return ret, err
		}
		ret.Notes = append(ret.Notes, note)
	}

	return ret, nil
}
