import Box from "@mui/material/Box";
import Header from "./Header";
import Drawer from "./Drawer";

export default function DashboardLayout() {
  return (
    <Box sx={{ display: "flex", width: "100%" }}>
      <Header />
      <Drawer />
    </Box>
  );
}
