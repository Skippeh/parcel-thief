import * as React from "react";
import InfoText from "../../../../../components/info_text";
import * as Dialog from "../../../../../components/dialog";
import * as Form from "../../../../../components/form";
import { styled } from "styled-components";
import SaveButton from "../../../../../components/save_button";
import {
  ApiResponse,
  FormErrors,
  MappedFormErrors,
  mapFormErrors,
} from "../../../../../services";
import { FrontendAccount, LocalAccount } from "../../../../../api_types";
import { createLocalAccountFor } from "../../../../../services/accounts_service";

const FormSubmit = styled(Form.Submit)`
  margin-top: 0.3rem;
`;

const FormRoot = styled(Form.Root)`
  width: 300px;
`;

interface Props {
  account: FrontendAccount;
  setLocalAccount: (localAccount: LocalAccount) => void;
}

const CreateLocalAccountButton = ({ account, setLocalAccount }: Props) => {
  const [open, setOpen] = React.useState(false);
  const [username, setUsername] = React.useState("");
  const [password, setPassword] = React.useState("");
  const [passwordConfirm, setPasswordConfirm] = React.useState("");
  const [formErrors, setFormErrors] = React.useState<MappedFormErrors | null>(
    null
  );
  const [error, setError] = React.useState<string | null>(null);

  function checkPasswordConfirm(value: string, formData: FormData) {
    return value !== formData.get("password");
  }

  async function onCreate(): Promise<ApiResponse<LocalAccount>> {
    const response = await createLocalAccountFor(
      account.id,
      username,
      password
    );

    if (response.data != null) {
      setLocalAccount(response.data);
      setOpen(false);
    } else if (response.error != null) {
      setFormErrors(mapFormErrors(response.formErrors));
      setError(response.error);
    }

    return response;
  }

  function clearFormErrors() {
    if (formErrors != null) {
      setFormErrors(null);
      setError(null);
    }
  }

  return (
    <>
      <Dialog.Root open={open} onOpenChange={setOpen}>
        <Dialog.Trigger>Create local account</Dialog.Trigger>
        <Dialog.Portal>
          <Dialog.Overlay />
          <Dialog.Content>
            <Dialog.Title>Create local account</Dialog.Title>
            <FormRoot autoComplete="off" onClearServerErrors={clearFormErrors}>
              <Form.Field
                name="username"
                serverInvalid={formErrors?.username != null}
              >
                <Form.Label>Username</Form.Label>
                <Form.Control
                  type="text"
                  name="username"
                  autoComplete="off"
                  required
                  onChange={(e) => setUsername(e.target.value)}
                />
                {formErrors?.username?.usernameExists && (
                  <Form.Message>The username is taken</Form.Message>
                )}
              </Form.Field>
              <Form.Field name="password">
                <Form.Label>Password</Form.Label>
                <Form.Control
                  type="password"
                  autoComplete="new-password"
                  required
                  onChange={(e) => setPassword(e.target.value)}
                />
              </Form.Field>
              <Form.Field name="passwordConfirm">
                <Form.Label>Confirm password</Form.Label>
                <Form.Control
                  type="password"
                  autoComplete="new-password"
                  required
                  onChange={(e) => setPasswordConfirm(e.target.value)}
                />
                <Form.Message match={checkPasswordConfirm}>
                  Passwords do not match
                </Form.Message>
              </Form.Field>
              <Dialog.Buttons>
                <SaveButton isForm saveAction={onCreate}>
                  Create
                </SaveButton>
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
