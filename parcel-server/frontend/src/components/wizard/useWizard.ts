import * as React from "react";

import WizardContext from "./wizardContext";

const useWizard = () => {
  const context = React.useContext(WizardContext);

  if (!context) {
    throw Error("Wrap your step with `Wizard`");
  }

  return context;
};

export default useWizard;
