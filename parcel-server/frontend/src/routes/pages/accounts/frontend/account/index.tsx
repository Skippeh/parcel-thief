import * as React from "react";
import { useState } from "react";
import { useParams } from "react-router-dom";
import { FrontendAccount, FrontendPermissions } from "../../../../../api_types";
import { getFrontendAccount } from "../../../../../services/accounts_service";
import * as Form from "../../../../../components/form";
import { styled } from "styled-components";
import { Info } from "@phosphor-icons/react";
import InfoText from "../../../../../components/info_text";
import PermissionsEditor from "./permissions_editor";

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

  function updatePermissions(
    accountId: number,
    permissions: FrontendPermissions[]
  ) {
    if (account == null) {
      return;
    }

    setAccount({
      ...account,
      permissions,
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
              updatePermissions={(permissions) =>
                updatePermissions(account.id, permissions)
              }
            />
          </Section>
          {account.localAccount && (
            <Section title="Local account">
              <Form.Root></Form.Root>
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
            {account.localAccount == null && (
              <>
                <button>Create local account</button>
                <InfoText title="Creating a local account">
                  <p>
                    Creating a local account allows a player to log in to the
                    frontend without needing to go through a provider log in.
                  </p>
                  <p>
                    At the moment this is required for players using Epic Games
                    Launcher who want to log in to the frontend.
                  </p>
                </InfoText>
              </>
            )}
          </div>
        </>
      )}
    </Wrapper>
  );
};

export default FrontendAccount;
