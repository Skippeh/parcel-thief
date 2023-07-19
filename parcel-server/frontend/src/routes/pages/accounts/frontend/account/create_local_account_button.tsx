import * as React from "react";
import InfoText from "../../../../../components/info_text";
import * as Dialog from "../../../../../components/dialog";
import * as Form from "../../../../../components/form";
import { styled } from "styled-components";
import PasswordInput from "../../../../../components/password_input";

const FormSubmit = styled(Form.Submit)`
  margin-top: 0.3rem;
`;

const FormRoot = styled(Form.Root)`
  width: 300px;
`;

const CreateLocalAccountButton = () => {
  const [open, setOpen] = React.useState(false);

  function onSubmit(ev: React.FormEvent<HTMLFormElement>) {
    ev.preventDefault();

    const formData = new FormData(ev.currentTarget);
    console.log(formData);
  }

  function checkPasswordConfirm(value: string, formData: FormData) {
    return value !== formData.get("password");
  }

  return (
    <>
      <Dialog.Root open={open} onOpenChange={setOpen}>
        <Dialog.Trigger>Create local account</Dialog.Trigger>
        <Dialog.Portal>
          <Dialog.Overlay />
          <Dialog.Content>
            <Dialog.Title>Create local account</Dialog.Title>
            <FormRoot onSubmit={onSubmit}>
              <Form.Field name="username">
                <Form.Label>Username</Form.Label>
                <Form.Control
                  type="text"
                  name="username"
                  autoComplete="off"
                  required
                />
              </Form.Field>
              <Form.Field name="password">
                <Form.Label>Password</Form.Label>
                <Form.Control
                  type="password"
                  autoComplete="new-password"
                  required
                  minLength={8}
                />
                <Form.Message match="tooShort">
                  Password must be at least 8 characters long
                </Form.Message>
              </Form.Field>
              <Form.Field name="passwordConfirm">
                <Form.Label>Confirm password</Form.Label>
                <Form.Control type="password" autoComplete="new-password" />
                <Form.Message match={checkPasswordConfirm}>
                  Passwords do not match
                </Form.Message>
              </Form.Field>
              <Dialog.Buttons>
                <FormSubmit>Create</FormSubmit>
                <Dialog.Close className="secondary">Cancel</Dialog.Close>
              </Dialog.Buttons>
            </FormRoot>
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
