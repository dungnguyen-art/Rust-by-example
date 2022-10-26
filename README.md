# fa-3_4

# **Ứng dụng Mail Client sử dụng giao thức SMTP, POP3, IMAP thông qua máy chủ Gmail và Outlook.**




## Các chức năng đã đạt được.

 - [Gửi Mail thông qua Gmail](https://awesomeopensource.com/project/elangosundar/awesome-README-templates)
 - [Gửi Mail thông qua Outlook](https://github.com/matiassingers/awesome-readme)
 - [Gửi Mail đính kèm tệp thông qua Gmail](https://bulldogjob.com/news/449-how-to-write-a-good-readme-for-your-github-project)
 - [Gửi Mail đính kèm tệp thông qua Outlook](https://bulldogjob.com/news/449-how-to-write-a-good-readme-for-your-github-project)
 - [Xuất Mail ra têp epl thông qua giao thức POP3](https://bulldogjob.com/news/449-how-to-write-a-good-readme-for-your-github-project)




## Quá trình phát triển sản phẩm


| STT |     MSV- Họ Tên                 | Các nội dung thực hiện                      | Thể hiện            |Ghi chú      |
| :-- | :-------------------------------| :------------------------------------------ |:---------------------------------------------- |:----------|
| `1` | `B19DCCN306 - Lê Nhật Huy`      | 21-5/10/2022: thực hiện gửi gmail bằng java ||**SendEmail.java**|||
|     |                                 | 5-12/10/2022: lưu mail gửi/ nhận vào database|| **Email.java**|||
|     |                                 | 12-19/10/2022: Thực hiện lưu mail/ import mail vào excel  với Apache Poi| **ImportMail.java**|||
| `2` | `B19DCCN708 - Nguyễn Văn Trưởng`| 21-5/10/2022: Thực hiện nhận và đọc gmail bằng java.||**ReceiveEmail.java**|||
|     |                                 | 5-12/10/2022: lấy email từ database.||**ReceiveMailOutlook.java**|||
|     |                                 | 12-19/10/2022:config outlook, gửi và nhận mail trong outlook.||**SentMailOutlook.java**|||
| `3` | `B19DCCN126 - Nguyễn Tiến Dũng` | 5-12/10/2022: nghiên cứu tìm hiểu nhận mail bằng IMAP/POP3||**ReceiveEmail.java**|||
|     |                                 | 12-19/2022: Tổng hợp lý thuyết smtp, pop3, imap||**Readme.md**|||
