import { useState, useEffect } from "react";
import CSS from "csstype";

// TODO: can i just use json instead of defining this?
interface TableEntry {
    video_id: string;
    title: string;
    channel_id: string;
    published: string;
}

const columns = [
    { label: "ID", accessor: "video_id" },
    { label: "Date", accessor: "published" },
    { label: "Channel", accessor: "channel_id" },
    { label: "Title", accessor: "title" },
];

export default function Table() {
    const [entries, setEntries] = useState<TableEntry[]>([]);
    const [sortMode, setSortMode] = useState("published"); // Sort by date published by default.
    const [reverseSortOrder, setReverseSortOrder] = useState<boolean>(false);

    async function refreshEntries() {
        try {
            const response = await fetch("http://127.0.0.1:8080/api/unwatched_videos");
            if (!response.ok) {
                throw new Error(`Response status: ${response.status}`);
            }

            setEntries(await response.json());
        } catch (error) {
            console.error(error);
        }
    }

    // Request data to populate the table with.
    useEffect(() => {
        refreshEntries();
    }, []);

    // Don't render if there's no data to display.
    if (entries.length === 0) {
        return null;
    }

    // Sort entries (see onClickHeader)
    let sortedEntries = entries.sort((a: TableEntry, b: TableEntry) => {
        if (reverseSortOrder) {
            [a, b] = [b, a];
        }
        return a[sortMode as keyof TableEntry].localeCompare(
            b[sortMode as keyof TableEntry]
        );
    });

    // Make the column headers feel like buttons.
    const thStyle: CSS.Properties = {
        cursor: "pointer",
        userSelect: "none",
    };

    // Implement alternating row colors.
    function trStyle(rowIndex: number) {
        return {
            backgroundColor: rowIndex % 2 === 0 ? "#444" : "black",
        };
    }

    //
    // Handle user input in these functions.
    //
    function onClickHeader(accessor: string, previousSortOrder: boolean) {
        if (sortMode === accessor) {
            setReverseSortOrder(!previousSortOrder);
        } else {
            setSortMode(accessor);
            setReverseSortOrder(false);
        }
    }
    function onClickDelete(entryId: string) {
        // Dummy API request to delete video. TODO
        console.log("begin video delete", entryId);
        setTimeout(() => {
            console.log("end video delete");
            refreshEntries();
        }, 1000);
    }
    function onClickWatch(entryId: string) {
        window.open("https://www.youtube.com/watch?v=" + entryId);
        onClickDelete(entryId);
    }

    // borderCollapse is needed to color rows correctly.
    return <table style={{ borderCollapse: "collapse", margin: "auto" }}>
        <thead>
            <tr>
                {columns.map(({ label, accessor }) => {
                    return <th onClick={() => onClickHeader(accessor, reverseSortOrder)} style={thStyle} key={accessor}>
                        {label}
                    </th>;
                })}
            </tr>
        </thead>
        <tbody>
            {sortedEntries.map((entry, rowIndex) => {
                return (
                    <tr key={entry.video_id} style={trStyle(rowIndex)}>
                        {columns.map(({ accessor }) => {
                            return <td key={accessor} style={{ "paddingRight": "10px" }}>
                                {entry[accessor as keyof TableEntry]}
                            </td>;
                        })}
                        <td>
                            <button title="Watch" onClick={() => onClickWatch(entry.video_id)}>
                                👁️
                            </button>
                            <button title="Delete" onClick={() => onClickDelete(entry.video_id)}>
                                ❌
                            </button>
                        </td>
                    </tr>
                );
            })}
        </tbody>
    </table>;
}