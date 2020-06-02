package users

import (
	"errors"
	"fmt"
	"regexp"
	"strings"

	"github.com/asaskevich/govalidator"
)

// ValidateFirstName validate a first name
func ValidateFirstName(firstName string) error {
	firstNameLen := len(firstName)

	if firstNameLen == 0 {
		return errors.New("first_name cannot be empty")
	}

	if firstNameLen > FIRST_NAME_MAX_LENGTH {
		return errors.New("first_name is too long")
	}

	return nil
}

// ValidateLastName validates a last name
func ValidateLastName(lastName string) error {
	lastNameLen := len(lastName)

	if lastNameLen == 0 {
		return errors.New("last_name cannot be empty")
	}

	if lastNameLen > LAST_NAME_MAX_LENGTH {
		return errors.New("last_name is too long")
	}

	return nil
}

// ValidateBio validate a user Bio
func ValidateBio(bio string) error {
	if len(bio) > BIO_MAX_LENGTH {
		return errors.New("bio is too long")
	}

	return nil
}

// ValidateDisplayName validates a displayName
func ValidateDisplayName(displayName string) error {
	displayNameLen := len(displayName)

	if displayNameLen == 0 {
		return errors.New("display_name cannot be empty")
	}

	if displayNameLen > DISPLAY_NAME_MAX_LENGTH {
		return errors.New("display_name is too long")
	}

	return nil
}

// TODO
/*
pub fn email<S: std::hash::BuildHasher>(
    disposable_emails: HashSet<String, S>,
    email: &str,
) -> Result<(), BloomError> {
    if email.is_empty() || !email.contains('@') {
        return Err(BloomError::Validation("email is not valid".to_string()));
    }

    let parts: Vec<&str> = email.split('@').collect();

    if parts.len() != 2 {
        return Err(BloomError::Validation("email is not valid".to_string()));
    }

    let user_part = parts[1];
    let domain_part = parts[0];

    if user_part.is_empty() || domain_part.is_empty() {
        return Err(BloomError::Validation("email is not valid".to_string()));
    }

    if email.trim() != email {
        return Err(BloomError::Validation(
            "email must not contains whitesapces".to_string(),
        ));
    }

    if email.len() > 128 {
        return Err(BloomError::Validation("email is too long".to_string()));
    }

    let user_re = Regex::new(r"^(?i)[a-z0-9.!#$%&'*+/=?^_`{|}~-]+\z")
        .expect("error compiling user email regex");
    let domain_re = Regex::new(
        r"(?i)^[a-z0-9](?:[a-z0-9-]{0,61}[a-z0-9])?(?:.[a-z0-9](?:[a-z0-9-]{0,61}[a-z0-9])?)*$",
    )
    .expect("error compiling domain email regex");

    if !user_re.is_match(user_part) {
        return Err(BloomError::Validation("email is not valid".to_string()));
    }
    if !domain_re.is_match(domain_part) {
        return Err(BloomError::Validation("email is not valid".to_string()));
    }

    if disposable_emails.contains(&domain_part.to_string()) {
        return Err(BloomError::Validation(
            "email domain is not valid".to_string(),
        ));
    }

    return Ok(());
}
*/

// ValidateEmail validates an email
func ValidateEmail(email string, disposableEmailDomains map[string]bool) error {
	if !govalidator.IsEmail(email) {
		return errors.New("email is not valid")
	}
	return nil
}

var isAlphaNumeric = regexp.MustCompile(`^[a-z0-9]+$`).MatchString

// ValidateUsername validates an username
func ValidateUsername(username string) error {
	usernameLength := len(username)

	if usernameLength == 0 {
		return errors.New("username cannot be empty")
	}

	if usernameLength < USERNAME_MIN_LENGTH {
		return fmt.Errorf("username must be longer than %d characters", USERNAME_MIN_LENGTH-1)
	}

	if usernameLength > USERNAME_MAX_LENGTH {
		return fmt.Errorf("username must be longer than %d characters", USERNAME_MAX_LENGTH)
	}

	if username != strings.ToLower(username) {
		return errors.New("username must be lowercase")
	}

	if !isAlphaNumeric(username) {
		return errors.New("username must contains only alphanumeric characters")
	}

	if strings.Contains(username, "admin") ||
		strings.Contains(username, "bloom") ||
		strings.HasSuffix(username, "bot") ||
		stringSliceContains(INVALID_USERNAMES, username) {
		return errors.New("username is not valid")
	}

	return nil
}

func stringSliceContains(s []string, e string) bool {
	for _, a := range s {
		if a == e {
			return true
		}
	}
	return false
}