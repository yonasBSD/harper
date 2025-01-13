import { registerPlugin } from '@wordpress/plugins';
import { PluginSidebar, PluginSidebarMoreMenuItem } from '@wordpress/editor';
import SidebarControl from './SidebarControl';
import Logo from './Logo';

import './index.css';
import React from 'react';

function Sidebar() {
	return (
		<>
			<PluginSidebarMoreMenuItem target="harper-sidebar" icon={ Logo }>
				Harper
			</PluginSidebarMoreMenuItem>
			<PluginSidebar name="harper-sidebar" title="Harper" icon={ Logo }>
				<SidebarControl />
			</PluginSidebar>
		</>
	);
}

// @ts-ignore
if ( ! window.__harperSidebarRegistered ) {
	registerPlugin( 'harper-sidebar', { render: Sidebar } );
	// @ts-ignore
	window.__harperSidebarRegistered = true;
}
