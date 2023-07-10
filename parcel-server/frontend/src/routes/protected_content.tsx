import * as React from "react";
import useSession from "../hooks/use_session";
import { UserPermissions } from "../context/session_context";
import { Outlet, useNavigate } from "react-router-dom";

interface Props extends React.PropsWithChildren {
  permissions?: UserPermissions;
}

function hasPermissions(
  userPermissions: UserPermissions,
  targetPermissions: UserPermissions
): boolean {
  return (userPermissions & targetPermissions) == targetPermissions;
}

/**
 * This component can be used to limit access to a specific route or child components.
 *
 * If used without any children, an Outlet will be rendered to show any sub routes.
 */
const ProtectedContent: React.FC<Props> = ({ children, permissions }) => {
  const { getUser } = useSession();
  const navigate = useNavigate();

  React.useEffect(() => {
    // Only redirect to login if there's no children
    if (children == null) {
      const user = getUser();

      if (user == null) {
        navigate("/login");
      }
    }
  }, []);

  const user = getUser();

  if (
    user == null ||
    (permissions != null && !hasPermissions(user.permissions, permissions))
  ) {
    // If children are specified, don't render anything when we don't have access.
    if (children != null) {
      return null;
    }

    // Render generic "no access" if this is used without any children (such as a base route)
    return <p>You lack the permissions to view this page.</p>;
  }

  return children == null ? <Outlet /> : children;
};

export default ProtectedContent;
