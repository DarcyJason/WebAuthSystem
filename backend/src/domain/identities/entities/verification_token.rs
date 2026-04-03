use crate::domain::common::value_objects::time::time_stamp::Timestamp;
use crate::domain::identities::value_objects::user::user_id::UserId;
use crate::domain::identities::value_objects::verification_token::verification_token_kind::VerificationTokenKind;
use crate::domain::identities::value_objects::verification_token::verification_token_value::VerificationTokenValue;
use crate::domain::identities::value_objects::verification_token::verification_used::VerificationTokenUsed;

pub struct VerificationToken {
    value: VerificationTokenValue,
    user_id: UserId,
    kind: VerificationTokenKind,
    used: VerificationTokenUsed,
    created_at: Timestamp,
    expires_at: Timestamp,
}
