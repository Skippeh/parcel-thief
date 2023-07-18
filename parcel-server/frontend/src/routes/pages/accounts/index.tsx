import * as React from "react";
import * as Tabs from "../../../components/tabs";
import FrontendAccountsTable from "./frontend_accounts_table";
import { useState } from "react";
import { FrontendAccountListItem } from "../../../api_types";
import { getAccounts } from "../../../services/accounts_service";

const Accounts = () => {
  const [frontendAccounts, setFrontendAccounts] = useState<
    FrontendAccountListItem[] | null | undefined
  >(undefined);

  React.useEffect(() => {
    (async () => {
      if (frontendAccounts == null) {
        const response = await getAccounts("frontend");

        if (response.data != null) {
          setFrontendAccounts(response.data.accounts);
        }
      }
    })();
  });

  return (
    <>
      <Tabs.Root defaultValue="frontend">
        <Tabs.List>
          <Tabs.Trigger value="frontend">Frontend accounts</Tabs.Trigger>
          <Tabs.Trigger value="game">Game accounts</Tabs.Trigger>
        </Tabs.List>
        <Tabs.Content value="frontend" forceMount>
          <FrontendAccountsTable accounts={frontendAccounts} />
        </Tabs.Content>
        <Tabs.Content value="game" forceMount>
          game
        </Tabs.Content>
      </Tabs.Root>
    </>
  );
};

export default Accounts;
