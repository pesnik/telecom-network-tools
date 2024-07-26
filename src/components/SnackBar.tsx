import * as React from "react";
import Snackbar from "@mui/material/Snackbar";
import IconButton from "@mui/material/IconButton";
import CloseIcon from "@mui/icons-material/Close";

interface Props {
  status: Number;
  message: String;
}

const SimpleSnackbar: React.FC<Props> = ({ status, message }) => {
  console.log(status, message);
  const [open, setOpen] = React.useState(true);

  const handleClose = (
    _event: React.SyntheticEvent | Event,
    reason?: string,
  ) => {
    if (reason === "clickaway") {
      return;
    }

    setOpen(false);
  };

  const action = (
    <React.Fragment>
      <IconButton
        size="small"
        aria-label="close"
        color={status === 0 ? "success" : "error"}
        onClick={handleClose}
      >
        <CloseIcon fontSize="small" />
      </IconButton>
    </React.Fragment>
  );

  return (
    <div>
      <Snackbar
        open={open}
        autoHideDuration={2000}
        onClose={handleClose}
        message={
          status === 0
            ? "Successfully Completed Dependency Check and File Saved"
            : message
        }
        action={action}
      />
    </div>
  );
};

export default SimpleSnackbar;
