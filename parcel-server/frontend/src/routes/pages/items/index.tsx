import * as React from "react";
import ContentBox from "../../layout/content_box";
import PageTitle from "../../../components/page_title";
import Table from "./table";
import { useState } from "react";
import { getSharedCargo } from "../../../services/baggages_service";
import { BaggageListItem } from "../../../api_types";

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
        <Table items={items} />
      </ContentBox>
    </div>
  );
};

export default Items;
