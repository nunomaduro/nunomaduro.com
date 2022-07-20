window.NewsletterForm = function NewsletterForm() {
    return {
        data: {
            email: '',
        },
        submitting: false,
        submitting_message: 'You will receive a confirmation email shortly.',
        submit() {
            this.submitting = true

            const email = this.data.email
            this.data.email = ''

            fetch('api/v1/newsletter', {
                method: 'POST',
                headers: {
                    'Content-Type': 'application/json',
                    Accept: 'application/json',
                },
                body: JSON.stringify({
                    email,
                }),
            })

            setTimeout(() => {
                this.submitting = false
            }, 4000)
        },
    }
}
