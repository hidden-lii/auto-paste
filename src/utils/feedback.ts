import { useConfirm, useSnackbar } from 'vuetify-use-dialog';

export function useFeedback() {
	const createConfirm = useConfirm();
	const createSnackbar = useSnackbar();

	function showConfirm(
		content: string,
		title: string,
		confirmationText = '确认',
		cancellationText = '取消'
	) {
		return createConfirm({
			content,
			title,
			confirmationText,
			cancellationText,
			dialogProps: {
				persistent: true
			}
		});
	}

	function showSnackbar(
		text: string,
		color: string,
		timeout = 1000,
		showCloseButton = false
	) {
		createSnackbar({
			text,
			snackbarProps: {
				timeout,
				color,
				minWidth: 'fit-content',
				maxWidth: 'fit-content'
			},
			showCloseButton
		});
	}

	return { showConfirm, showSnackbar };
}
