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
import * as Tabs from "../../../../../components/tabs";

const FormRoot = styled(Form.Root)`
  max-width: 350px;
`;

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
          <Tabs.Root defaultValue="permissions">
            <Tabs.List>
              <Tabs.Trigger value="permissions">Permissions</Tabs.Trigger>
              <Tabs.Trigger value="localAccount">Local account</Tabs.Trigger>
              <Tabs.Trigger value="providerConnection">
                Provider connection
              </Tabs.Trigger>
            </Tabs.List>
            <Tabs.Content value="permissions" $padded>
              <PermissionsEditor
                permissions={account.permissions}
                accountId={account.id}
                updatePermissions={updatePermissions}
              />
            </Tabs.Content>
            <Tabs.Content value="localAccount" $padded>
              <>
                {account.localAccount && (
                  <>
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
                    <div className="buttons">
                      <ResetPasswordButton
                        account={account}
                        // always prompt current password if user is trying to reset their own password
                        // server side also checks that the user has permission to reset other account's passwords if relevant
                        promptCurrentPassword={account.id == sessionAccountId}
                      />
                    </div>
                  </>
                )}
                {account.localAccount == null && (
                  <>
                    <p>
                      No credentials are currently associated with this account.
                    </p>
                    <div className="buttons">
                      <CreateLocalAccountButton
                        account={account}
                        setLocalAccount={updateLocalAccount}
                      />
                    </div>
                  </>
                )}
              </>
            </Tabs.Content>
            <Tabs.Content value="providerConnection" $padded>
              {account.providerConnection && (
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
              )}
              {account.providerConnection == null && (
                <p>
                  No provider connection is currently associated with this
                  account.
                </p>
              )}
            </Tabs.Content>
          </Tabs.Root>
        </>
      )}
    </Wrapper>
  );
};

export default FrontendAccount;
