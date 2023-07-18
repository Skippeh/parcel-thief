import * as React from "react";
import { useState } from "react";
import { useParams } from "react-router-dom";
import PageTitle from "../../../../components/page_title";
import ContentBox from "../../../layout/content_box";
import { FrontendAccount } from "../../../../api_types";
import { getFrontendAccount } from "../../../../services/accounts_service";

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

  return (
    <>
      <PageTitle>Edit Frontend Account</PageTitle>
      <ContentBox></ContentBox>
    </>
  );
};

export default FrontendAccount;
