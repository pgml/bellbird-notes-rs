pub const DEFAULT_STYLE: &str = "
window {
	background-color: #F0F0F0;
	color: #222;
	border-top: 1px solid #DBDBDB;
	font-size: 13px;
}

text {
	background-color: #fff;
	padding: 8px;
	border-radius: 4px;
}

popover contents { border-radius: 5px; }
popover modelbutton {
	min-height: 27px;
	padding-left: 28px;
}

.dialogue { border-top: 0; }
.dialogue .button { background-color: #FEFEFE; }
.dialogue button label { font-weight: 400; }

windowcontrols button {
	-gtk-icon-size: 12px;
	min-width: 18px;
	min-height: 18px;
	padding-top: 8px;
	padding-right: 2px;
	padding-left: 8px;
}

windowcontrols button:nth-child(2) { padding-left: 2px; }

headerbar {
	background-color: #f0f0f0;
	box-shadow: 0;
}

.directories-panel {
	/* border-right: 1px solid #DBDBDB; */
	/* border-right: 1px solid #EAEAEA; */
}

.directories-panel > label,
windowhandle label {
	text-transform: uppercase;
	font-weight: 700;
	color: #A5A1BC;
}

.directories-tree .tree-row-wrapper.hide {
	display: none;
	height: 0;
}

.directories-panel listview {
	background-color: transparent;
	padding: 0 10px;
}

.directories-panel row { border-radius: 4px; }
.directories-panel treeexpander.hide { opacity: 0; }

.notes-panel {
	border-radius: 4px;
	background-color: #fff;
	/* border-right: 4px solid #f0f0f0; */
	border: 1px solid #EDEDED;
}

.notes-panel listview { padding: 0 5px; }

.notes-panel > box > label {
	text-transform: uppercase;
	font-weight: 700;
	color: #A5A1BC;
}

.notes-panel row {
	border-radius: 4px;
	border-bottom-color: #eee;
}

.notes-panel row:selected image {
	filter: invert(100%) opacity(25%);
}

.notes-panel row:last-child {
	border-bottom-color: transparent;
}

.editor-panel {
	border-radius: 4px;
	background-color: #fff;
	border: 1px solid #EDEDED;
}

.editor-panel textview {
	line-height: 1.2;
}

#breadcrumb {
	color: #999;
	font-size: 12px;
}

#breadcrumb .icon {
	filter: invert(100%) brightness(150%) contrast(60%);
}

#breadcrumb .note-name {
	font-weight: 700;
}

.status-bar {
	background-color: #E4E5E7;
}
";
