class MyBook : Book
{
public:
    explicit MyBook(string title, string author, int price)
        : Book{title, author}, price(price) {}

    void display() override
    {
        cout << "Title: " << title << endl
             << "Author: " << author << endl
             << "Price: " << price << endl;
    }

private:
    int price;
};
