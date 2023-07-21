import * as React from "react";
import * as Dialog from "../../../../../components/dialog";
import * as Form from "../../../../../components/form";
import FrontendAccount from ".";
import SaveButton, {
  CooldownDelay,
} from "../../../../../components/save_button";
import { ApiResponse } from "../../../../../services";
import { resetAccountPassword } from "../../../../../services/accounts_service";

interface Props {
  account: FrontendAccount;
  promptCurrentPassword?: boolean;
}

const ResetPasswordButton = ({ account, promptCurrentPassword }: Props) => {
  const [open, setOpen] = React.useState(false);
  const [currentPassword, setCurrentPassword] = React.useState("");
  const [newPassword, setNewPassword] = React.useState("");
  const [confirmPassword, setConfirmPassword] = React.useState("");
  const [error, setError] = React.useState<string | null>(null);

  const resetPassword = async (): Promise<ApiResponse<unknown>> => {
    const response = await resetAccountPassword(
      account.id,
      currentPassword,
      newPassword
    );

    if (response.statusCode == 200) {
      setTimeout(() => {
        setOpen(false);
      }, CooldownDelay);
    } else if (response.error != null) {
      setError(response.error);
    }

    return response;
  };

  return (
    <Dialog.Root open={open} onOpenChange={setOpen}>
      <Dialog.Trigger>Reset password</Dialog.Trigger>
      <Dialog.Portal>
        <Dialog.Overlay />
        <Dialog.Content>
          <Dialog.Title>Reset password</Dialog.Title>
          <Form.Root>
            {promptCurrentPassword && (
              <Form.Field name="currentPassword">
                <Form.Label>Current password</Form.Label>
                <Form.Control
                  type="password"
                  required
                  value={currentPassword}
                  onChange={(e) => setCurrentPassword(e.target.value)}
                />
              </Form.Field>
            )}
            <Form.Field name="newPassword">
              <Form.Label>New password</Form.Label>
              <Form.Control
                type="password"
                required
                autoComplete="new-password"
                value={newPassword}
                onChange={(e) => setNewPassword(e.target.value)}
              />
            </Form.Field>
            <Form.Field name="newPasswordConfirm">
              <Form.Label>Password confirmation</Form.Label>
              <Form.Control
                type="password"
                required
                autoComplete="new-password"
                value={confirmPassword}
                onChange={(e) => setConfirmPassword(e.target.value)}
              />
            </Form.Field>
            <span className="error">{error}</span>
            <Dialog.Buttons>
              <SaveButton isForm saveAction={resetPassword}>
                Save
              </SaveButton>
              <Dialog.Close className="secondary">Cancel</Dialog.Close>
            </Dialog.Buttons>
          </Form.Root>
        </Dialog.Content>
      </Dialog.Portal>
    </Dialog.Root>
  );
};

export default ResetPasswordButton;
