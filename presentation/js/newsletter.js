window.NewsletterForm = function NewsletterForm() {
    return {
        data: {
            email: '',
        },
        submitting: false,
        submitting_message: 'You will receive a confirmation email shortly.',
        submit() {
            this.submitting = true

            fetch('api/v1/newsletter', {
                method: 'POST',
                headers: {
                    'Content-Type': 'application/json',
                    Accept: 'application/json',
                },
                body: JSON.stringify(this.data),
            }).finally(() => {
                this.data.email = ''
            })

            setTimeout(() => {
                this.submitting = false
            }, 4000)
        },
    }
}
