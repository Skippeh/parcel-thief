import * as React from "react";
import InfoText from "../../../../../components/info_text";
import * as Dialog from "../../../../../components/dialog";

const CreateLocalAccountButton = () => {
  const [open, setOpen] = React.useState(false);

  return (
    <>
      <Dialog.Root open={open} onOpenChange={setOpen}>
        <Dialog.Trigger>Create local account</Dialog.Trigger>
        <Dialog.Portal>
          <Dialog.Overlay />
          <Dialog.Content>
            <Dialog.Title>Create local account</Dialog.Title>

            <Dialog.Buttons>
              <button className="primary">Create</button>
              <Dialog.Close className="secondary">Cancel</Dialog.Close>
            </Dialog.Buttons>
          </Dialog.Content>
        </Dialog.Portal>
      </Dialog.Root>
      <InfoText title="Creating a local account">
        <p>
          Creating a local account allows a player to log in to the frontend
          without needing to go through a provider log in.
        </p>
        <p>
          At the moment this is required for players using Epic Games Launcher
          who want to log in to the frontend.
        </p>
      </InfoText>
    </>
  );
};

export default CreateLocalAccountButton;
