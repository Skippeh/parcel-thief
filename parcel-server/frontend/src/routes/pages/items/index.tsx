import * as React from "react";
import ContentBox from "../../layout/content_box";
import PageTitle from "../../../components/page_title";
import Table from "./table";
import { useState } from "react";
import { getSharedCargo } from "../../../services/baggages_service";
import { BaggageListItem } from "../../../api_types";
import * as Tabs from "../../../components/tabs";

const Items = () => {
  const [items, setItems] = useState<BaggageListItem[] | undefined | null>();

  React.useEffect(() => {
    // fetch items if there are no items yet
    (async () => {
      if (items == null) {
        const response = await getSharedCargo();

        if (response.data != null) {
          setItems(response.data.baggages);
        } else {
          console.error(response.statusCode, response.error);
          setItems(null);
        }
      }
    })();
  }, []);

  return (
    <div>
      <PageTitle>Items</PageTitle>
      <ContentBox>
        <Tabs.Root defaultValue="sharedCargo">
          <Tabs.List>
            <Tabs.Trigger value="sharedCargo">Shared cargo</Tabs.Trigger>
            <Tabs.Trigger value="lostCargo">Lost cargo</Tabs.Trigger>
            <Tabs.Trigger value="wastedCargo">Wasted cargo</Tabs.Trigger>
          </Tabs.List>
          <Tabs.Content value="sharedCargo">
            <Table items={items} />
          </Tabs.Content>
          <Tabs.Content value="lostCargo">Lost cargo</Tabs.Content>
          <Tabs.Content value="wastedCargo">Wasted cargo</Tabs.Content>
        </Tabs.Root>
      </ContentBox>
    </div>
  );
};

export default Items;
