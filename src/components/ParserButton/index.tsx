import React, { useState } from "react";
import Button from "@mui/material/Button";
import { useFileContext } from "contexts/FileContext";
import { invoke } from "@tauri-apps/api/tauri";
import SimpleSnackbar from "components/SnackBar";

const FileParserButton: React.FC = () => {
  const { selectedFile } = useFileContext();
  const [fileParsed, setFileParsed] = useState<boolean>(false);

  const sendFileForParsing = async () => {
    setFileParsed(false);
    if (!selectedFile) return;

    try {
      await invoke("parse_and_find_dependencies", {
        filePath: selectedFile,
      });
      setFileParsed(true);
    } catch (error) {
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
      {fileParsed ? <SimpleSnackbar /> : <></>}
    </>
  );
};

export default FileParserButton;
