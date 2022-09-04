import type { FunctionComponent } from "react";

/**
 * Component for an Icon that represents a back button.
 * Icon sourced from: https://fonts.google.com/icons
 */
export const BackIcon: FunctionComponent = () => (
    <svg xmlns="http://www.w3.org/2000/svg" height="24px" viewBox="0 0 24 24" width="24px" fill="#999"
         className="icon home-icon">
        <path d="M0 0h24v24H0V0z" fill="none" opacity=".87"/>
        <path d="M17.51 3.87L15.73 2.1 5.84 12l9.9 9.9 1.77-1.77L9.38 12l8.13-8.13z"/>
    </svg>
)

/**
 * Component for an Icon that represents a home button.
 * Icon sourced from: https://fonts.google.com/icons
 */
export const HomeIcon: FunctionComponent = () => (
    <svg xmlns="http://www.w3.org/2000/svg" height="24px" viewBox="0 0 24 24" width="24px" fill="#999"
         className="icon home-icon">
        <path d="M0 0h24v24H0V0z" fill="none"/>
        <path d="M10 20v-6h4v6h5v-8h3L12 3 2 12h3v8h5z"/>
    </svg>
)

/**
 * Component for an Icon that represents a file.
 * Icon sourced from: https://fonts.google.com/icons
 */
export const FileIcon: FunctionComponent = () => (
    <svg xmlns="http://www.w3.org/2000/svg" height="32px" viewBox="0 0 24 24" width="32px" fill="#999"
         className="icon home-icon">
        <path d="M0 0h24v24H0V0z" fill="none"/>
        <path d="M14 2H4v20h16V8l-6-6zm2 16H8v-2h8v2zm0-4H8v-2h8v2zm-3-5V3.5L18.5 9H13z"/>
    </svg>
)

/**
 * Component for an Icon that represents a folder.
 * Icon sourced from: https://fonts.google.com/icons
 */
export const FolderIcon: FunctionComponent = () => (
    <svg xmlns="http://www.w3.org/2000/svg" height="32px" viewBox="0 0 24 24" width="32px" fill="#999"
         className="icon home-icon">
        <path d="M0 0h24v24H0V0z" fill="none"/>
        <path d="M10 4H2v16h20V6H12l-2-2z"/>
    </svg>
)
