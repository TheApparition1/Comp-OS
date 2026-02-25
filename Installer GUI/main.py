

import tkinter as tk
from tkinter import ttk


class FirstBootWizard(tk.Tk):
    def __init__(self):
        super().__init__()

        self.title("InstConf Setup Wizard")
        self.geometry("600x400")
        self.resizable(False, False)

        self.username = tk.StringVar()
        self.timezone = tk.StringVar(value="Australia/Sydney")
        self.desktop = tk.StringVar(value="GNOME")
        self.install_wine = tk.BooleanVar()
        self.install_dev = tk.BooleanVar()

        self.container = ttk.Frame(self, padding=20)
        self.container.pack(fill="both", expand=True)

        self.pages = []
        self.current_index = 0

        # Create pages
        self.pages.append(self.page_welcome())
        self.pages.append(self.page_user())
        self.pages.append(self.page_desktop())
        self.pages.append(self.page_options())
        self.pages.append(self.page_finish())

        for page in self.pages:
            page.place(relx=0, rely=0, relwidth=1, relheight=1)

        # Navigation
        self.nav_frame = ttk.Frame(self)
        self.nav_frame.pack(fill="x", pady=10)

        self.back_button = ttk.Button(self.nav_frame, text="Back", command=self.back)
        self.back_button.pack(side="left", padx=10)

        self.next_button = ttk.Button(self.nav_frame, text="Next", command=self.next)
        self.next_button.pack(side="right", padx=21)

        self.show_page(self.current_index)

    def show_page(self, index):
        for i, page in enumerate(self.pages):
            page.lift() if i == index else page.lower()
        self.back_button["state"] = "normal" if index > 0 else "disabled"
        self.next_button["text"] = "Finish" if index == len(self.pages) - 1 else "Next"

    def next(self):
        if self.current_index < len(self.pages) - 1:
            self.current_index += 1
            self.show_page(self.current_index)
        else:
            self.destroy()

    def back(self):
        if self.current_index > 0:
            self.current_index -= 1
            self.show_page(self.current_index)

    # ----- Pages -----

    def page_welcome(self):
        frame = ttk.Frame(self.container)
        ttk.Label(frame, text="Welcome to InstConf", font=("Helvetica", 18)).pack(pady=20)
        ttk.Label(frame, text="This wizard will help you configure your system.").pack(pady=10)
        return frame

    def page_user(self):
        frame = ttk.Frame(self.container)
        ttk.Label(frame, text="Create Your User", font=("Helvetica", 16)).pack(pady=15)
        #-Use this code here for color changes-#



        ttk.Label(frame, text="Username:").pack(anchor="w")
        ttk.Entry(frame, textvariable=self.username).pack(fill="x", pady=5)

        ttk.Label(frame, text="Timezone:").pack(anchor="w")
        ttk.Entry(frame, textvariable=self.timezone).pack(fill="x", pady=5)
        return frame

    def page_desktop(self):
        frame = ttk.Frame(self.container)
        ttk.Label(frame, text="Select Desktop Environment", font=("Helvetica", 16)).pack(pady=15)

        for option in ["GNOME", "KDE", "XFCE", "Hyprland", "Minimal"]:
            ttk.Radiobutton(frame, text=option, value=option, variable=self.desktop).pack(anchor="w")
        return frame

    def page_options(self):
        frame = ttk.Frame(self.container)
        ttk.Label(frame, text="Additional Options", font=("Helvetica", 16)).pack(pady=15)
        #--#
        ttk.Checkbutton(frame, text="Install Wine (Windows app support)", variable=self.install_wine).pack(anchor="w", pady=5)
        ttk.Checkbutton(frame, text="Install Developer Tools", variable=self.install_dev).pack(anchor="w", pady=5)
        return frame

    def page_finish(self):
        frame = ttk.Frame(self.container)
        ttk.Label(frame, text="Ready to Configure!", font=("Helvetica", 16)).pack(pady=20)

        summary = (
            f"Username: {self.username.get()}\n"
            f"Timezone: {self.timezone.get()}\n"
            f"Desktop: {self.desktop.get()}\n"
            f"Wine: {'Yes' if self.install_wine.get() else 'No'}\n"
            f"Dev Tools: {'Yes' if self.install_dev.get() else 'No'}"
        )
        ttk.Label(frame, text=summary).pack(pady=10)
        return frame


if __name__ == "__main__":
    app = FirstBootWizard()
    app.mainloop()