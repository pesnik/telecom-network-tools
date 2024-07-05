import { createBrowserRouter } from "react-router-dom";

import MainRoutes from "./MainRoutes";

const router = createBrowserRouter([MainRoutes], {
  basename: import.meta.env.BASE_URL,
});

export default router;
