import React, { createContext, useContext, useMemo, useState } from "react";
import { createTheme, ThemeProvider } from "@mui/material/styles";

export type ColorMode = "light" | "dark";

interface ColorModeContextType {
  mode: ColorMode;
  toggleColorMode: () => void;
}

const initialContext: ColorModeContextType = {
  mode: "light",
  toggleColorMode: () => {},
};

const ColorModeContext = createContext<ColorModeContextType>(initialContext);

export const useColorMode = () => useContext(ColorModeContext);

interface ColorModeProviderProps {
  children: React.ReactNode;
}

export const ColorModeProvider: React.FC<ColorModeProviderProps> = ({
  children,
}) => {
  const [mode, setMode] = useState<ColorMode>("light");

  const toggleColorMode = () => {
    setMode((prevMode) => (prevMode === "light" ? "dark" : "light"));
  };

  const theme = useMemo(
    () =>
      createTheme({
        palette: {
          mode,
        },
      }),
    [mode]
  );

  const contextValue = useMemo(
    () => ({
      mode,
      toggleColorMode,
    }),
    [mode]
  );

  return (
    <ColorModeContext.Provider value={contextValue}>
      <ThemeProvider theme={theme}>{children}</ThemeProvider>
    </ColorModeContext.Provider>
  );
};
