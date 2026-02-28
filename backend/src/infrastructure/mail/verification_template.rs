use crate::domain::{
    auth::value_objects::verification_token::VerificationToken, common::time::ttl::TTL,
    user::value_objects::user_email::UserEmail,
};

pub fn build_verification_email(
    user_email: UserEmail,
    verification_token: VerificationToken,
    expires_seconds: TTL,
) -> String {
    format!(
        r#"
        <!DOCTYPE html>
        <html>
        <body style="margin:0;padding:0;background:#f4f7f9;font-family:-apple-system,BlinkMacSystemFont,'Segoe UI',Roboto,Helvetica,Arial,sans-serif;">
          <table width="100%" cellpadding="0" cellspacing="0" style="padding:60px 0;">
            <tr>
              <td align="center">
                <table width="520" cellpadding="0" cellspacing="0"
                       style="background:#ffffff;border-radius:24px;padding:48px;box-shadow:0 4px 12px rgba(0,0,0,0.05);">

                  <tr>
                    <td align="left" style="padding-bottom:32px;">
                      <div style="font-size:24px;font-weight:800;color:#1a1a1a;letter-spacing:-0.5px;">
                        Homeryland
                      </div>
                    </td>
                  </tr>

                  <tr>
                    <td style="font-size:16px;color:#4b5563;line-height:1.6;">
                      <h2 style="font-size:20px;color:#111827;margin-top:0;">Verify your email address</h2>
                      Hello <strong>{user_email}</strong>,<br/>
                      Please copy the unique verification token below to complete your authentication.
                    </td>
                  </tr>

                  <tr>
                    <td align="center" style="padding:40px 0;">
                      <div style="
                        background:#f8fafc;
                        border:2px dashed #e2e8f0;
                        border-radius:16px;
                        padding:24px;
                        text-align:center;">
                        <code style="
                          font-family:'SFMono-Regular',Consolas,'Liberation Mono',Menlo,monospace;
                          font-size:15px;
                          color:#2563eb;
                          weight:600;
                          word-break:break-all;
                          line-height:1.4;">
                          {verification_token}
                        </code>
                        <div style="margin-top:12px;font-size:12px;color:#94a3b8;font-weight:500;text-transform:uppercase;letter-spacing:1px;">
                          Security Token
                        </div>
                      </div>
                    </td>
                  </tr>

                  <tr>
                    <td style="font-size:14px;color:#6b7280;line-height:1.7;">
                      Note: This code will expire in <strong>{expires_minutes} minutes</strong>.
                      <br/>For your security, please do not forward this email to anyone.
                    </td>
                  </tr>

                  <tr>
                    <td style="padding-top:48px;border-top:1px solid #f1f5f9;margin-top:40px;">
                      <p style="font-size:12px;color:#9ca3af;line-height:1.5;margin:0;">
                        If you did not request this verification, please ignore this email.<br/>
                        © 2026 Homeryland. All rights reserved.
                      </p>
                    </td>
                  </tr>

                </table>
              </td>
            </tr>
          </table>
        </body>
        </html>
    "#,
        user_email = user_email,
        verification_token = verification_token,
        expires_minutes = expires_seconds.value().as_secs() / 60
    )
}
