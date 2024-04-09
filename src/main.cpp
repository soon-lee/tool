#include <gtk/gtk.h>

static void on_close_clicked(GtkButton *button, gpointer user_data) {
    gtk_window_close(GTK_WINDOW(user_data));
}

void activate(GtkApplication *app, gpointer user_data) {
    // 创建主窗口
    auto app_window = gtk_application_window_new(app);
    gtk_window_set_title(GTK_WINDOW(app_window), "Custom Title bar Example");
    gtk_window_set_default_size(GTK_WINDOW(app_window), 600, 400);

    // 创建并配置 HeaderBar
    GtkWidget *header_bar = gtk_header_bar_new();
//    gtk_header_bar_set_decoration_layout(GTK_HEADER_BAR(header_bar), "Custom Title");
    gtk_header_bar_set_show_title_buttons(GTK_HEADER_BAR(header_bar), TRUE); // 显示默认关闭按钮

    // 添加自定义按钮到 HeaderBar
    GtkWidget *close_button = gtk_button_new_from_icon_name("window-close-symbolic");
    g_signal_connect(close_button, "clicked", G_CALLBACK(on_close_clicked), app_window);
    gtk_header_bar_pack_end(GTK_HEADER_BAR(header_bar), close_button);

    // 设置窗口的标题栏为自定义 HeaderBar
    gtk_window_set_titlebar(GTK_WINDOW(app_window), header_bar);

    // 显示主窗口
//    gtk_widget_show(app_window);
    gtk_window_present (GTK_WINDOW (app_window));
}

int main(int argc, char **argv) {
    // 初始化 GTK 应用
    GtkApplication *app = gtk_application_new("org.example.Custom-Title-bar", G_APPLICATION_DEFAULT_FLAGS);
    g_signal_connect(app, "activate", G_CALLBACK(activate), NULL);

    int status = g_application_run(G_APPLICATION(app), argc, argv);
    g_object_unref(app);

    return status;
}