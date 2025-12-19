import HourglassIcon from 'virtual:icons/lucide/hourglass';
import ScrollTextIcon from 'virtual:icons/lucide/scroll-text';
import SettingsIcon from 'virtual:icons/lucide/settings';
import BarChartIcon from 'virtual:icons/lucide/bar-chart';
import TrendingUpIcon from 'virtual:icons/lucide/trending-up';

export const SIDEBAR_ROUTES = {
	'/main/stats': { label: 'Statistics', icon: BarChartIcon },
	'/main/charts': { label: 'DPS Charts', icon: TrendingUpIcon },
	'/main/history': { label: 'History (WIP)', icon: HourglassIcon },
	'/main/changelog': { label: 'Changelog (WIP)', icon: ScrollTextIcon },
	'/main/settings': { label: 'Settings', icon: SettingsIcon }
};
