import * as React from "react";
import * as Dialog from "../../../../../components/dialog";
import FrontendAccount from ".";

interface Props {
  account: FrontendAccount;
}

const ResetPasswordButton = ({ account }: Props) => {
  return (
    <Dialog.Root>
      <Dialog.Trigger>Reset password</Dialog.Trigger>
      <Dialog.Portal>
        <Dialog.Overlay />
        <Dialog.Content>
          <Dialog.Title>Reset password</Dialog.Title>
          <Dialog.Close className="secondary">Cancel</Dialog.Close>
        </Dialog.Content>
      </Dialog.Portal>
    </Dialog.Root>
  );
};

export default ResetPasswordButton;
