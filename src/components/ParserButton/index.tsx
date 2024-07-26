import React, { useState } from "react";
import Button from "@mui/material/Button";
import { useFileContext } from "contexts/FileContext";
import { invoke } from "@tauri-apps/api/tauri";
import SimpleSnackbar from "components/SnackBar";

interface Response {
  status: String;
  message: String;
}

const FileParserButton: React.FC = () => {
  const { selectedFile } = useFileContext();
  const [fileParsed, setFileParsed] = useState<boolean>(false);
  const [responseMessage, setResponseMessage] = useState<string>("");
  const [error, setError] = useState<Number>(0);

  const sendFileForParsing = async () => {
    setFileParsed(false);
    setResponseMessage("");
    setError(0);
    if (!selectedFile) return;

    try {
      const responseString = await invoke<string>(
        "parse_and_find_dependencies",
        {
          filePath: selectedFile,
        },
      );

      const response: Response = JSON.parse(responseString);
      console.log(response);
      if (response.status === "0") {
        setFileParsed(true);
        setResponseMessage(response.message as string);
        setError(0);
      } else {
        setResponseMessage(response.message as string);
        setError(response.status as Number);
      }
      setFileParsed(true);
    } catch (error) {
      setError(1);
      setResponseMessage(error as string);
      console.error("Error parsing file:", error);
    }
  };

  return (
    <>
      <Button
        disabled={!selectedFile}
        variant="contained"
        onClick={sendFileForParsing}
      >
        Parse Dependency
      </Button>
      {fileParsed ? (
        <SimpleSnackbar message={responseMessage} status={error} />
      ) : (
        <></>
      )}
    </>
  );
};

export default FileParserButton;
