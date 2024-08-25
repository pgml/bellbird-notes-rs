pub const DEFAULT_STYLE: &str = "
window {
	background-color: #F0F0F0;
	color: #222;
	border-top: 1px solid #DBDBDB;
}

headerbar {
	background-color: #f0f0f0;
	box-shadow: 0;
}

.directories-panel {
	/* border-right: 1px solid #DBDBDB; */
}

.directories-panel > label {
	text-transform: uppercase;
	font-weight: 700;
	color: #A5A1BC;
}

.directories-panel listview {
	background-color: transparent;
}

.directories-panel row {
	border-radius: 4px;
}

.notes-panel {
	border-radius: 4px;
	background-color: #fff;
	/* border-right: 1px solid #DBDBDB; */
}

.notes-panel > label {
	text-transform: uppercase;
	font-weight: 700;
	color: #A5A1BC;
}

.notes-panel row {
	border-radius: 4px;
	border-bottom-color: #eee;
}

.notes-panel row:last-child {
	border-bottom-color: transparent;
}

.editor-panel {
	border-radius: 4px;
	background-color: #fff;
}

.status-bar {
	background-color: #E4E5E7;
}
";
