import * as React from "react";
import { useState } from "react";
import { useParams } from "react-router-dom";
import {
  FrontendAccount,
  FrontendPermissions,
  LocalAccount,
} from "../../../../../api_types";
import { getFrontendAccount } from "../../../../../services/accounts_service";
import * as Form from "../../../../../components/form";
import { styled } from "styled-components";
import PermissionsEditor from "./permissions_editor";
import CreateLocalAccountButton from "./create_local_account_button";
import ResetPasswordButton from "./reset_password_button";
import useSession from "../../../../../hooks/use_session";

const FormRoot = styled(Form.Root)`
  max-width: 350px;
`;

const SectionWrapper = styled.div`
  &:first-child h2 {
    margin-top: 0;
  }
`;

interface SectionProps {
  title: string;
}

const Section = ({
  title,
  children,
}: React.PropsWithChildren<SectionProps>) => {
  return (
    <SectionWrapper>
      <h2>{title}</h2>
      {children}
    </SectionWrapper>
  );
};

const Wrapper = styled.div`
  & .buttons {
    margin-top: 1.5rem;

    & > *:first-child {
      margin-left: 0;
    }
  }
`;

const FrontendAccount = () => {
  const { id } = useParams();
  const [loadError, setLoadError] = useState<string | null>(null);
  const [account, setAccount] = useState<FrontendAccount | null | undefined>(
    undefined
  );
  const { getJwtPayload } = useSession();
  const { accountId: sessionAccountId } = getJwtPayload()!; // payload will never be null at this point

  React.useEffect(() => {
    (async () => {
      if (account == null && id != null) {
        const id_num = parseInt(id);
        const response = await getFrontendAccount(id_num);

        if (response.data != null) {
          setAccount(response.data);
        } else {
          if (response.statusCode != 404) {
            setLoadError(response.error);
          }

          setAccount(null);
        }
      }
    })();
  });

  function updatePermissions(permissions: FrontendPermissions[]) {
    if (account == null) {
      return;
    }

    setAccount({
      ...account,
      permissions,
    });
  }

  function updateLocalAccount(localAccount: LocalAccount) {
    if (account == null) {
      return;
    }

    setAccount({
      ...account,
      localAccount,
    });
  }

  return (
    <Wrapper>
      {loadError != null && <p>{loadError}</p>}
      {account === null && loadError == null && <p>Account not found</p>}
      {account === undefined && <p>Loading...</p>}
      {account != null && (
        <>
          <Section title="Permissions">
            <PermissionsEditor
              permissions={account.permissions}
              accountId={account.id}
              updatePermissions={updatePermissions}
            />
          </Section>
          {account.localAccount && (
            <Section title="Local Account">
              <FormRoot>
                <Form.Field name="username">
                  <Form.Label>Username</Form.Label>
                  <Form.Control
                    readOnly
                    type="text"
                    value={account.localAccount.username}
                  />
                </Form.Field>
              </FormRoot>
            </Section>
          )}
          {account.providerConnection && (
            <Section title="Provider Connection">
              <FormRoot>
                <Form.Field name="providerName">
                  <Form.Label>Type</Form.Label>
                  <Form.Control
                    readOnly
                    type="text"
                    value={account.providerConnection.provider}
                  />
                </Form.Field>
                <Form.Field name="providerId">
                  <Form.Label>Identity</Form.Label>
                  <Form.Control
                    readOnly
                    type="text"
                    value={account.providerConnection.providerId}
                  />
                </Form.Field>
              </FormRoot>
            </Section>
          )}
          <div className="buttons">
            {account.localAccount != null && (
              <ResetPasswordButton
                account={account}
                // always prompt current password if user is trying to reset their own password
                // server side also checks that the user has permission to reset other account's passwords if relevant
                promptCurrentPassword={account.id == sessionAccountId}
              />
            )}
            {account.localAccount == null && (
              <>
                <CreateLocalAccountButton
                  account={account}
                  setLocalAccount={updateLocalAccount}
                />
              </>
            )}
          </div>
        </>
      )}
    </Wrapper>
  );
};

export default FrontendAccount;
