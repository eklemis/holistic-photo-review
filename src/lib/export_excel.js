import { utils, writeFileXLSX } from 'xlsx';

export const exportToExcel = (rows, sheet_name, opt) => {
	const ws = utils.json_to_sheet(rows);
	const wb = utils.book_new();
	utils.book_append_sheet(wb, ws, sheet_name);

	writeFileXLSX(wb, `Review List - ${opt}.xlsx`);
};
